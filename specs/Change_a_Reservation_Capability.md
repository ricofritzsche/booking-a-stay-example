# Change a Reservation Capability

The **Change a Reservation** capability covers the moment when an existing confirmed reservation should be adjusted.

A guest may want to change the dates of a stay or the number of guests. In some cases, a host or the platform may also change a reservation, depending on the rules of the product. The system has to decide whether the requested change can be accepted. If the new booking situation is valid, the reservation is changed. If not, the existing reservation remains as it is.

The capability is not about editing a reservation record. It is about replacing one confirmed booking situation with another valid booking situation.

## Domain language

A **reservation** is the confirmed result of a successful booking.

A **reservation change** is a requested adjustment to an existing confirmed reservation.

A **current stay** is the stay already confirmed by the existing reservation.

A **changed stay** is the stay requested by the change.

A **requester** is the actor asking for the change. This can be the guest, the host, or the platform.

A **night** is the unit of occupancy. A stay from July 1 to July 4 occupies July 1, July 2, and July 3. The check-out day is not occupied.

## Business purpose

The purpose of this capability is to change an existing reservation without losing the correctness of the booking situation.

A confirmed reservation already occupies nights for a listing. Changing it may release some nights and occupy others. The system must therefore evaluate the changed stay as a booking decision, but with one important difference: the existing reservation itself should not block its own change.

The capability answers one business question:

> Can this confirmed reservation be changed to the requested stay?

If the answer is yes, the reservation is changed. If the answer is no, the change is rejected with a clear business reason, and the reservation remains unchanged.

## Change request

A reservation change request contains the information needed to evaluate the new stay:

```text
reservation
requester
new check-in date
new check-out date
new guest count
change time
optional reason
```

The request expresses the intent to change an existing reservation. It does not guarantee that the change can be accepted.

The system still has to evaluate whether the reservation exists, whether it is still confirmed, whether the requester is allowed to change it, and whether the changed stay is valid for the guest, the listing, and the requested nights.

## What the capability has to evaluate

To decide whether the reservation can be changed, the capability needs to evaluate the current reservation and the requested new stay.

It must know whether the reservation exists. A missing reservation cannot be changed.

It must know whether the reservation is still confirmed. A cancelled reservation cannot be changed as if it were still active.

It must know who requests the change. A guest may be allowed to change their own reservation. A host may be allowed to change reservations for their listing only under specific rules. The platform may change a reservation for administrative reasons.

It must check whether the reservation is still changeable at the requested time. In the first version, this can be simple: a reservation can be changed before check-in, but not after the stay has already started.

It must know whether the guest is still eligible to book. A blocked guest cannot move an existing reservation to a new booking situation in the first version.

It must know whether the listing is still bookable. A disabled listing cannot accept the changed stay, even if the requested nights are free.

It must evaluate the changed stay against the listing’s rules. The new date range must be valid, the guest count must be allowed, and the stay length must fit the listing’s minimum and maximum nights.

It must also check availability for the new requested nights. Nights already occupied by the same reservation should be ignored for this decision. Other confirmed reservations or blocked nights still matter.

## Change conditions

A reservation can be changed only when all required conditions are true:

```text
the reservation exists
the reservation is currently confirmed
the requester is allowed to change it
the reservation is still changeable
the guest is still eligible to book
the listing is still bookable
the changed date range is valid
the changed guest count is valid
the changed stay respects the listing’s rules
the requested nights are available, excluding the reservation being changed
```

If one of these conditions is not true, the reservation is not changed.

This keeps the business meaning clear. The system does not partially change the reservation. Either the changed stay is accepted as a valid new booking situation, or the existing reservation remains unchanged.

## Rejection reasons

A reservation change should be rejected with a clear business reason.

Typical rejection reasons are:

```text
reservation not found
reservation already cancelled
requester is not allowed to change this reservation
reservation can no longer be changed
guest is blocked
listing is disabled
invalid date range
too many guests
stay is too short
stay is too long
listing is unavailable for the requested dates
```

The rejection reason belongs to the domain. It should explain why the change was not accepted, not expose how the system stores reservations.

## Effect on availability

Changing a reservation affects the booking situation of the listing.

Assume a reservation currently occupies:

```text
July 1
July 2
July 3
```

The guest requests a change to:

```text
July 3
July 4
July 5
```

The system must treat July 3 carefully. It is already occupied by the same reservation, so it should not block the change. But July 4 and July 5 must be checked against other reservations and host blocks.

If the change is accepted, the reservation no longer occupies July 1 and July 2. It now occupies July 3, July 4, and July 5.

If the change is rejected, nothing changes. The reservation continues to occupy July 1, July 2, and July 3.

## Successful outcome

When the change is accepted, the system changes the reservation.

The changed reservation records the new booking situation:

```text
reservation
guest
listing
new check-in date
new check-out date
new guest count
change time
status: confirmed
```

From the domain perspective, the important result is that the guest still has a confirmed reservation, but the stay now follows the changed details.

## Business examples

A guest has a reservation from July 1 to July 4 for two guests. The guest changes the stay to July 2 to July 5. The listing allows the new stay, the requested nights are available, and the guest is allowed to change the reservation. The system changes the reservation.

A guest tries to change a reservation from July 1 to July 4 to July 3 to July 6, but another confirmed reservation already occupies July 5. The system rejects the change because the listing is already reserved for part of the requested stay.

A guest tries to increase the guest count from two to five, but the listing allows a maximum of four guests. The system rejects the change because there are too many guests.

A guest tries to change a reservation that was already cancelled. The system rejects the request because a cancelled reservation cannot be changed.

A guest tries to change the reservation after the stay has already started. In the first version, the system rejects the request because the reservation can no longer be changed.

A platform operator changes a reservation for an administrative reason. If platform changes are allowed by the rules, the system accepts the change and records the changed reservation situation.

## What this capability does not cover

This capability changes the stay details of a reservation. It does not handle every consequence of a real reservation change.

Price recalculation, payment adjustments, refunds, cancellation fees, host approval, guest messaging, notification emails, tax changes, payout changes, and policy-specific change windows are outside the first version unless they directly affect whether the reservation may be changed.

These concerns can become separate capabilities later. The Change a Reservation capability should stay focused on the change decision: whether this confirmed reservation can move to the requested booking situation.

## Relation to other capabilities

Change a Reservation is closely related to **Book a Stay**, because the changed stay must be evaluated against the same booking situation: guest eligibility, listing bookability, listing rules, and availability.

It is also related to **Cancel a Reservation**, because changing a reservation may release nights that were previously occupied.

It remains separate from **Manage Listing Availability**. A host can block nights independently of reservations. A reservation change must respect those blocks, but it does not define them.

It remains separate from **Manage Booking Eligibility**. A guest may be blocked from making new reservations, and a listing may be disabled from receiving bookings. These conditions can affect whether a reservation may be changed, but they are controlled by a different capability.

## Capability summary

The Change a Reservation capability accepts a requested adjustment only when the changed booking situation is valid.

It starts with the intent to change an existing reservation. It evaluates the current reservation, the requester, the timing of the change, the changed stay, the listing rules, and the availability of the requested nights. If the change is valid, the reservation is changed. If not, the reservation remains unchanged and the request is rejected with a clear business reason.

The capability is important because it shows that a reservation is not just confirmed once. Its booking situation can change, but only when the new situation is still valid.
