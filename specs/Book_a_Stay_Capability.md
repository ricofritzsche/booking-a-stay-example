# Book a Stay Capability

The **Book a Stay** capability covers the moment when a guest wants to reserve a listing for a specific date range.

The guest chooses a listing, a check-in date, a check-out date, and the number of guests. The system then has to decide whether this stay can be confirmed. If the guest is allowed to book, the listing can receive bookings, the requested nights are available, and the listing’s booking rules are satisfied, the system confirms a reservation.

The capability is not about creating a record. It is about making a booking decision.

A guest does not ask the system to “insert a reservation”. A guest wants to book a stay. The business result of a successful booking is a **confirmed reservation**.

## Domain language

A **guest** is the person who wants to book the stay.

A **listing** is the place offered for booking, for example an apartment, house, private room, or similar accommodation.

A **stay** describes the guest’s intended visit. It includes the listing, check-in date, check-out date, and guest count.

A **reservation** is the confirmed result of a successful booking.

A **night** is the unit of occupancy. A stay from July 1 to July 4 occupies the nights of July 1, July 2, and July 3. The check-out day itself is not occupied.

A **bookable listing** is a listing that can currently receive reservations.

An **eligible guest** is a guest who is currently allowed to book.

## Business purpose

The purpose of this capability is to protect the booking decision.

A reservation should only be confirmed when the booking situation still allows it. This means the system must not confirm a reservation just because the request looks valid in isolation. It has to evaluate the request against the current booking situation of the guest, the listing, and the requested nights.

The capability answers one business question:

> Can this guest book this listing for this date range with this number of guests?

If the answer is yes, the reservation is confirmed. If the answer is no, the booking is rejected with a clear business reason.

## Booking request

A booking request contains the information needed to evaluate the stay:

```text
guest
listing
check-in date
check-out date
guest count
```

The request must describe a real stay. The check-in date must be before the check-out date. The guest count must be greater than zero. The requested stay must fit the rules of the listing.

The check-in date must not be in the past. In the first version, same-day booking is allowed. A stay can start today or in the future, but not before the current booking date.

The request itself does not guarantee that a reservation can be confirmed. It only expresses the guest’s intent.

## What the capability has to evaluate

To decide whether the stay can be booked, the capability needs to evaluate several parts of the booking situation.

It must know whether the guest exists and is allowed to book. A blocked guest cannot confirm a new reservation, even if the listing is available.

It must know whether the listing exists and can receive bookings. A disabled listing cannot be booked, even if the requested nights are free.

It must check the listing’s booking rules. A listing may define a maximum guest count, a minimum number of nights, or a maximum stay length.

It must check whether the stay can still be booked at the current booking date. A stay cannot be confirmed when the check-in date is already in the past.

It must check whether the requested nights are available. A reservation cannot be confirmed when another confirmed reservation already occupies one of the requested nights or the host has blocked one of the requested nights.

These checks belong together because they all influence the same business decision.

## Confirmation conditions

A reservation can be confirmed only when all required conditions are true:

```text
the guest exists
the guest is eligible to book
the listing exists
the listing is bookable
the date range is valid
the check-in date is not in the past
the guest count is valid
the stay respects the listing’s rules
none of the requested nights are unavailable because of a confirmed reservation or host block
```

If one of these conditions is not true, the reservation is not confirmed.

This is important because availability alone is not enough. A listing can have free nights and still be disabled. A guest can request valid dates and still be blocked. A stay can fit the calendar and still exceed the allowed guest count.

## Rejection reasons

A booking rejection should explain why the reservation could not be confirmed.

Typical rejection reasons are:

```text
guest not found
guest is blocked
listing not found
listing is disabled
invalid date range
stay starts in the past
too many guests
stay is too short
stay is too long
listing is unavailable for the requested dates
```

The rejection reason belongs to the domain. It should not expose technical details. The guest or caller needs to know what prevented the booking, not how the system stores reservations.

## Availability by night

Availability is evaluated per night.

For example, a stay from July 1 to July 4 occupies three nights:

```text
July 1
July 2
July 3
```

The check-out date, July 4, is not occupied by this reservation. Another guest may check in on July 4 if the listing is otherwise available.

This distinction matters because many booking mistakes come from treating the end date as occupied. In stay booking, the check-out day ends the stay. It does not block another stay from starting on the same day.

## Successful outcome

When the booking can be accepted, the system confirms a reservation.

The confirmed reservation records the business result:

```text
reservation
guest
listing
check-in date
check-out date
guest count
confirmation time
booking terms used for confirmation
status: confirmed
```

From the domain perspective, the important result is not that data was written. The important result is that the guest now has a confirmed reservation and the requested nights are no longer available for another overlapping reservation.

The confirmed reservation records the relevant booking terms used for the decision, such as maximum guests, minimum nights, and maximum nights. This keeps the reservation understandable even if the listing rules change later.

## Business examples

A guest books a listing from July 1 to July 4 for two guests. The guest is eligible, the listing is bookable, the listing allows two guests, and no confirmed reservation overlaps the requested nights. The system confirms the reservation.

A guest tries to book the same listing from July 3 to July 6. July 3 is already occupied by the first reservation. The system rejects the booking because the listing is already reserved for part of the requested stay.

A blocked guest tries to book a free listing. The nights are available, but the guest is not allowed to book. The system rejects the booking because the guest is blocked.

A guest tries to book a listing for dates that have already passed. The listing may be available and the guest may be eligible, but the system rejects the booking because the stay starts in the past.

A guest tries to book a listing for six people, but the listing allows a maximum of four guests. The nights may be available, but the stay violates the listing’s rules. The system rejects the booking because there are too many guests.

A guest books a listing for five guests while the listing allows five guests. Later the host reduces the maximum guest count to four. The existing reservation remains valid because it was confirmed under the earlier booking terms. The reservation records those terms so the decision remains understandable later.

A guest books from July 4 to July 7 after another guest checks out on July 4. This can be valid because the first reservation does not occupy the check-out day.

## What this capability does not cover

This capability confirms a reservation. It does not handle every concern of a real booking platform.

Payment authorization, pricing, taxes, cancellation policies, host approval workflows, guest communication, refunds, reviews, and fraud checks are outside the first version unless they directly affect whether the reservation can be confirmed.

These concerns can become separate capabilities later. The Book a Stay capability should stay focused on the booking decision: whether this guest can reserve this listing for this date range.

## Relation to other capabilities

The booking situation can be changed by other capabilities.

Cancelling a reservation can release nights that were previously occupied.

Changing a reservation can release old nights and reserve new nights.

Managing listing availability can block or release nights independently from guest reservations.

Managing booking eligibility can block a guest from making new reservations or disable a listing from receiving bookings.

These capabilities are separate because their intent is different. But they influence the same booking situation. Book a Stay must therefore rely on the current state of guest eligibility, listing bookability, listing rules, and availability for the requested nights.

## Capability summary

The Book a Stay capability confirms a reservation only when the full booking situation allows it.

It starts with the guest’s intent to book a stay. It evaluates the guest, the listing, the requested date range, the current booking date, the guest count, the listing rules, and the availability of the requested nights. If all conditions are valid, the reservation is confirmed. If not, the booking is rejected with a clear business reason.

The capability is intentionally small, but it is not trivial. It shows the central nature of stay booking: a reservation is only valid when the decision behind it was made against the correct booking situation.
