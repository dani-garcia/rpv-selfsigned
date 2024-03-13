
# This probably only works on emulators or rooted devices
#adb root

# To get the hash: `openssl x509 -inform PEM -subject_hash_old -in badssl.com.ca.pem | head -1`
#subjectHash=86b3d5cf

# This copies the certificate to the correct path, but seems to not be detected correctly in my emulator
#adb push ./badssl.com.ca.pem /data/misc/user/0/cacerts-added/$subjectHash.0
#adb shell "su 0 chmod 644 /data/misc/user/0/cacerts-added/$subjectHash.0"
#adb shell "su 0 chown system:system /data/misc/user/0/cacerts-added/$subjectHash.0"
#adb reboot

adb push ./badssl.com.ca.pem /sdcard/Download/

# To install the copied certificate manually
# - Open the Settings app
# - Scroll down and go to Security & privacy
# - Scroll all the way down and go to More security & privacy
# - Tap on Encryption & credentials
# - Tap on Install a certificate, then on CA certificate
# - Tap on Install anyway
# - Go to the Download folder and select the certificate
