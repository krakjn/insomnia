#include <IOKit/pwr_mgt/IOPMLib.h>
#include <CoreFoundation/CoreFoundation.h>
#include <stdio.h>

static IOPMAssertionID assertionID = 0;

bool set_assertion() {
    CFStringRef reasonForActivity = CFSTR("Preventing sleep for critical operation");
    IOReturn result = IOPMAssertionCreateWithName(
        kIOPMAssertionTypeNoDisplaySleep,
        kIOPMAssertionLevelOn,
        reasonForActivity,
        &assertionID
    );
    return result == kIOReturnSuccess;
}

void release_assertion() {
    if (IOPMAssertionRelease(assertionID) != kIOReturnSuccess) {
        printf("Failed to release system sleep assertion.\n");
    }
}