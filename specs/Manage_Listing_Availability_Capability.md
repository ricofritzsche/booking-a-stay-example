# Manage Listing Availability Capability

The **Manage Listing Availability** capability covers the moment when a host controls whether a listing can receive reservations for specific nights.

A host may block nights because the listing is used privately, needs maintenance, is not ready for guests, or should simply not be bookable for that period. A host may also release previously blocked nights so that they can be booked again.

The capability is not about changing a listing description or editing a calendar record. It is about changing the booking situation of a listing.

## Domain language

A **host** is the person or organization offering a listing.

A **listing** is the place that can be booked by guests.

A **night** is the unit of availability. A stay from July 1 to July 4 occupies the nights of July 1, July 2, and July 3. The check-out day is not occupied.

An **available night** is a night that can be used for a new reservation, as long as the listing is bookable and no other rule prevents the booking.

A **blocked night** is a night the host has made unavailable for booking.

A **confirmed reservation** also occupies nights, but this is different from a host block. A reservation means a guest has a confirmed stay. A host block means the host has decided that the night should not be bookable.

## Business purpose

The purpose of this capability is to let the host control the listing’s calendar without confusing host availability with guest reservations.

A listing can be bookable in general and still have individual nights blocked. A listing can also have free nights but be disabled completely by another capability. Availability is therefore only one part of the booking situation.

The capability answers one business question:

> Can this host block or release these nights for this listing?

If the answer is yes, the listing availability is changed. If the answer is no, the request is rejected with a clear business reason.

## Availability request

An availability request contains the information needed to evaluate the change:

```text
host
listing
date range
requested availability change
reason
change time
```

The requested change can be:

```text
block nights
release blocked nights
```

The request expresses the host’s intent. It does not automatically mean the calendar can be changed. The system still has to evaluate whether the listing exists, whether the requester is allowed to manage it, and whether the requested change is valid for the selected nights.

## What the capability has to evaluate

To decide whether listing availability can be changed, the capability needs to evaluate the current listing situation.

It must know whether the listing exists. Availability cannot be managed for a missing listing.

It must know whether the requester is allowed to manage this listing. A host should not be able to change availability for a listing they do not control.

It must check whether the requested date range is valid. The start date must be before the end date.

It must know whether the requested nights are already occupied by confirmed reservations. In the first version, a host block should not be placed over an active confirmed reservation. The reservation already occupies the night for a different business reason.

It must know whether the requested nights are already blocked when the host wants to block them, or not blocked when the host wants to release them. This allows the system to reject meaningless changes or treat them as already satisfied, depending on the desired product behavior.

## Blocking conditions

Nights can be blocked only when the required conditions are true:

```text
the listing exists
the requester may manage the listing
the date range is valid
the requested nights are not occupied by confirmed reservations
the requested nights are not already blocked
```

If one of these conditions is not true, the nights are not blocked.

The important point is that blocking availability is a host decision. It should not overwrite or silently conflict with an existing guest reservation.

## Release conditions

Blocked nights can be released only when the required conditions are true:

```text
the listing exists
the requester may manage the listing
the date range is valid
the requested nights are currently blocked by the host
```

Releasing a blocked night only removes the host block. It does not guarantee that the night is available for a new reservation. Another confirmed reservation, a listing-level disablement, or another booking rule may still prevent booking.

This distinction matters because “released” does not mean “booked by nobody under every possible rule”. It only means the host block no longer prevents booking.

## Rejection reasons

An availability change should be rejected with a clear business reason.

Typical rejection reasons are:

```text
listing not found
requester is not allowed to manage this listing
invalid date range
night is already reserved
night is already blocked
night is not blocked
availability cannot be changed for this period
```

The rejection reason belongs to the domain. It should explain what prevented the availability change, not how the system stores calendar data.

## Effect on the booking situation

Managing listing availability directly affects future booking decisions.

If a host blocks July 1 to July 4, the blocked nights are:

```text
July 1
July 2
July 3
```

A guest cannot book a stay that needs one of these nights. A stay starting on July 4 may still be possible, because July 4 is the end of the blocked range and is not blocked by that range.

If the host later releases July 2, this only removes the host block for July 2. The night may become available, but only if no confirmed reservation or other rule prevents booking.

## Successful outcome

When the availability change is accepted, the system records the changed booking situation for the listing.

For blocked nights, the result is:

```text
listing
blocked nights
blocked by
block reason
block time
```

For released nights, the result is:

```text
listing
released nights
released by
release time
```

From the domain perspective, the important result is that future booking decisions will see the changed availability.

## Business examples

A host blocks a listing from July 1 to July 4 because of maintenance. No confirmed reservation occupies these nights. The system blocks July 1, July 2, and July 3.

A guest later tries to book the listing from July 2 to July 5. The system rejects the booking because one or more requested nights are blocked.

A host tries to block July 3, but there is already a confirmed reservation for that night. The system rejects the request because the night is already reserved.

A host releases July 2 after previously blocking it. The system removes the host block for that night.

A host releases July 2, but another confirmed reservation still occupies it. The host block is gone, but the night is still not available for a new booking.

A person tries to manage availability for a listing they do not own. The system rejects the request because the requester is not allowed to manage this listing.

## What this capability does not cover

This capability manages listing availability for specific nights. It does not manage all listing settings.

Changing the listing title, description, photos, amenities, address, pricing, minimum stay, maximum stay, guest capacity, or booking status belongs to other capabilities.

This capability also does not confirm reservations. It only changes whether specific nights are blocked or released by the host. The Book a Stay capability later uses this booking situation when deciding whether a reservation can be confirmed.

## Relation to other capabilities

Manage Listing Availability directly influences **Book a Stay**. A blocked night cannot be used for a new reservation.

It also influences **Change a Reservation**, because a changed reservation must respect the current availability of the requested nights.

It is related to **Cancel a Reservation**, but it is not the same. Cancelling a reservation removes the occupancy caused by that reservation. Releasing a host block removes a host decision. These are different business actions.

It remains separate from **Manage Booking Eligibility**. A listing may be generally bookable or disabled. Availability manages specific nights. Booking eligibility controls whether the listing can receive bookings at all.

## Capability summary

The Manage Listing Availability capability lets a host block or release nights for a listing.

It starts with the host’s intent to change the listing calendar. It evaluates the listing, the requester, the requested date range, existing reservations, and current host blocks. If the change is valid, the booking situation of the listing is changed. If not, the request is rejected with a clear business reason.

The capability is important because availability is not only the result of reservations. Hosts also need direct control over when a listing can and cannot receive bookings.
