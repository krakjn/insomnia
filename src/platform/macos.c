#include <IOKit/pwr_mgt/IOPMLib.h>
#include <CoreFoundation/CoreFoundation.h>
#include <stdio.h>

static IOPMAssertionID assertionID;

void set_assertion() {
    CFStringRef reasonForActivity = CFSTR("Preventing sleep for critical operation");

    // Create the assertion to prevent system sleep
    IOReturn result = IOPMAssertionCreateWithName(
        kIOPMAssertionTypeNoDisplaySleep,   // Assertion type
        kIOPMAssertionLevelOn,              // Assertion level
        reasonForActivity,                  // Reason for the assertion
        &assertionID                        // Store the assertion ID
    );

    if (result == kIOReturnSuccess) {
        printf("System sleep is prevented successfully. Assertion ID: %u\n", assertionID);
    } else {
        printf("Failed to create system sleep assertion.\n");
    }
}

void release_assertion() {
    IOReturn result = IOPMAssertionRelease(assertionID);
    if (result == kIOReturnSuccess) {
        printf("System sleep assertion released successfully.\n");
    } else {
        printf("Failed to release system sleep assertion.\n");
    }
}