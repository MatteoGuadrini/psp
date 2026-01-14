#! /usr/bin/env bash

VERSION=$(grep ^version Cargo.toml | awk '{print $3}' | tr -d '"')

if [ -n "$(which docker)" ]; then
    ccli="$(which docker)"
elif [ -n "$(which podman)" ]; then
	ccli="$(which podman)"
else
  echo "Docker or Podman is not installed"
  exit 1
fi

# Create temporary environment for build
mkdir -p /tmp/psp_rpm
mkdir -p /tmp/psp_deb

# Windows compile
$ccli run -it --rm -v $PWD:/tmp/psp "rust:1.80.1-slim" bash -c "
cd /tmp/psp
apt update
apt install -y g++-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu"

# Linux compile
$ccli run -it --rm -v $PWD:/tmp/psp "rust:1.80.1-slim" bash -c "
cd /tmp/psp
cargo build --release"

# Build rpm
cat >/tmp/psp_rpm/psp.spec <<EOL
Name:           psp
Version:        ${VERSION}
Release:        1
Summary:        (Python Scaffolding Projects)
BuildArch:      x86_64
License:        Apache License 2.0
URL:            https://github.com/MatteoGuadrini/psp
Source0:        %{name}-%{version}.tar.gz
Requires:       bash

%global _description %{expand:
PSP (Python Scaffolding Projects).}

%description %{_description}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/%{_bindir}
cp %{name} %{buildroot}/%{_bindir}

%files
%license LICENSE.md
%doc README.md
%{_bindir}/%{name}

%changelog
* %{__cat} CHANGES.md
EOL

$ccli run -it --rm -v $PWD:/tmp/psp -v /tmp/psp_rpm:/tmp/psp_rpm -e "VERSION=${VERSION}"  "rockylinux:9.3" bash -c "
dnf install rpmdevtools gcc -y
cd /tmp/psp_rpm
mkdir "psp-${VERSION}"
cp /tmp/psp/target/release/psp "psp-${VERSION}/"
cp /tmp/psp/LICENSE.md "psp-${VERSION}/"
cp /tmp/psp/README.md "psp-${VERSION}/"
cp /tmp/psp/CHANGES.md "psp-${VERSION}/"
tar --create --file "psp-${VERSION}.tar.gz" "psp-${VERSION}"
rpmdev-setuptree
cp "psp-${VERSION}.tar.gz" /root/rpmbuild/SOURCES/
cp psp.spec /root/rpmbuild/SPECS/
rpmbuild -bb /root/rpmbuild/SPECS/psp.spec
mv /root/rpmbuild/RPMS/x86_64/psp-${VERSION}*.x86_64.rpm psp.rpm"

# Build deb
mkdir -p /tmp/psp_deb/psp-${VERSION}
mkdir -p /tmp/psp_deb/psp-${VERSION}/DEBIAN
mkdir -p /tmp/psp_deb/psp-${VERSION}/usr/local/bin
cp $PWD/target/release/psp "/tmp/psp_deb/psp-${VERSION}/usr/local/bin/"
cat >/tmp/psp_deb/psp-${VERSION}/DEBIAN/control <<EOL
Package: psp
Version: ${VERSION}
Maintainer: Matteo Guadrini
Architecture: all
Description: PSP (Python Scaffolding Projects)
EOL

$ccli run -it --rm -v $PWD:/tmp/psp -v /tmp/psp_deb:/tmp/psp_deb -e "VERSION=${VERSION}" ubuntu:24.04 bash -c "
cd /tmp/psp_deb
dpkg-deb --build "psp-${VERSION}"
mv "psp-${VERSION}.deb" psp.deb"
