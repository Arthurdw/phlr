export VERSION=0.1.0

echo ""
echo "Fetching phlr v${VERSION} debian package..."
sudo wget https://github.com/Arthurdw/phlr/releases/download/v${VERSION}/phlr_${VERSION}_amd64.deb

echo ""
echo "Installing phlr v${VERSION}..."
sudo dpkg -i phlr_${VERSION}_amd64.deb

echo ""
echo "Removing downloaded phlr v${VERSION} debian file..."
sudo rm phlr_${VERSION}_amd64.deb

echo ""
echo "Successfully installed phlr v${VERSION}!"
phlr --help
