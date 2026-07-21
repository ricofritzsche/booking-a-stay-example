# Cancel a Reservation Capability

The **Cancel a Reservation** capability covers the moment when an existing confirmed reservation should no longer remain active.

A guest, host, or the platform may request the cancellation. The system has to decide whether the reservation can be canceled and what this means for the booking situation of the listing. If the cancellation is allowed, the reservation is marked as canceled and the nights occupied by this reservation are no longer blocked by it.

The capability is not about deleting a reservation. A cancellation is a business action. The reservation existed, it was confirmed, and later it was canceled. This history matters for the guest, the host, support, reporting, and later capabilities such as refunds or cancellation policies.

## Domain language

A **reservation** is the confirmed result of a successful booking.

A **canceled reservation** is a reservation that no longer gives the guest the right to stay and no longer blocks the listing through this reservation.

A **cancellation request** expresses the intent to cancel an existing reservation.

A **requester** is the actor who asks for the cancellation. This can be the guest, the host, or the platform.

A **cancellation reason** describes why the reservation was canceled. In the first version, this can be optional and simple. Later, it can become more important for policies, reporting, or support.

## Business purpose

The purpose of this capability is to end an active reservation in a controlled way.

A confirmed reservation occupies the requested nights of a listing. When the reservation is canceled, those nights are no longer occupied by this reservation. This does not automatically mean every night becomes bookable again, because the host may have blocked some nights separately or the listing itself may be disabled. But the canceled reservation should no longer prevent another stay from being booked.

The capability answers one business question:

> Can this requester cancel this reservation at this point in time?

If the answer is yes, the reservation is canceled. If the answer is no, the cancellation is rejected with a clear business reason.

## Cancellation request

A cancellation request contains the information needed to evaluate the cancellation:

```text
reservation
requester
cancellation time
optional cancellation reason
```

The request itself does not guarantee that the reservation can be canceled. It only expresses the intent to cancel.

The system still has to evaluate whether the reservation exists, whether it is currently confirmed, whether the requester is allowed to cancel it, and whether cancellation is still allowed for this reservation.

## What the capability has to evaluate

To decide whether the reservation can be canceled, the capability needs to evaluate the current reservation situation.

It must know whether the reservation exists. A reservation that does not exist cannot be canceled.

It must know whether the reservation is still confirmed. A reservation that is already canceled should not be canceled again.

It must know who requests the cancellation. A guest may cancel their own reservation. A host may be allowed to cancel reservations for their listing. The platform may cancel reservations for administrative or trust reasons.

It may also need to check whether the cancellation is still allowed at the requested time. In the first version, this rule can be simple. For example, cancellation may be allowed before check-in and rejected after the stay has already started. More detailed cancellation policies can be added later.

## Cancellation conditions

A reservation can be canceled only when the required conditions are true:

```text
the reservation exists
the reservation is currently confirmed
the requester is allowed to cancel it
the reservation is still cancellable at the requested time
```

If one of these conditions is not true, the reservation is not canceled.

This keeps cancellation separate from technical state changes. The important point is not that a status changes. The important point is that an active reservation stops being active for a valid business reason.

## Rejection reasons

A cancellation rejection should explain why the reservation could not be canceled.

Typical rejection reasons are:

```text
reservation not found
reservation already canceled
requester is not allowed to cancel this reservation
cancellation is no longer allowed
stay has already started
```

The rejection reason belongs to the domain. It should not expose technical details. The guest, host, platform operator, or caller needs to know what prevented the cancellation.

## Effect on availability

Cancelling a reservation affects the booking situation of the listing.

For example, a reservation from July 1 to July 4 occupies these nights:

```text
July 1
July 2
July 3
```

When this reservation is canceled, these nights are no longer occupied by this reservation. Another guest may be able to book them later, but only if the listing is bookable and no other reservation or host block prevents it.

This distinction matters. Cancellation does not directly say “these nights are available for everyone”. It says: this reservation no longer blocks them.

## Successful outcome

When the cancellation is accepted, the system cancels the reservation.

The canceled reservation records the business result:

```text
reservation
canceled by
cancellation time
optional cancellation reason
status: canceled
```

From the domain perspective, the important result is that the guest no longer has a confirmed stay and the reservation no longer occupies the listing’s nights.

## Business examples

A guest cancels their confirmed reservation for July 1 to July 4 before check-in. The reservation exists, it is still confirmed, and the guest is allowed to cancel it. The system cancels the reservation.

A guest tries to cancel a reservation already canceled earlier. The system rejects the request because the reservation is already canceled.

A guest tries to cancel another guest’s reservation. The system rejects the request because the requester is not allowed to cancel this reservation.

A host cancels a reservation for one of their listings. If host cancellation is allowed in the current rules, the system cancels the reservation. If not, the cancellation is rejected.

A platform operator cancels a reservation because of an administrative decision. The system cancels the reservation and records the platform as the requester.

A guest tries to cancel after the stay has already started. In the first version, this can be rejected because cancellation after check-in is outside the simple cancellation rule.

## What this capability does not cover

This capability cancels a reservation. It does not handle every consequence of a real cancellation.

Refunds, cancellation fees, payout adjustments, guest notifications, host penalties, dispute handling, insurance, and policy-specific deadlines are outside the first version unless they directly affect whether the reservation may be canceled.

These concerns can become separate capabilities later. The Cancel a Reservation capability should stay focused on the cancellation decision: whether this requester can cancel this reservation at this point in time.

## Relation to other capabilities

Cancelling a reservation influences the same booking situation that **Book a Stay** depends on.

Book a Stay checks whether requested nights are already occupied by confirmed reservations. Once a reservation is canceled, it should no longer block those nights.

Change a Reservation may also depend on cancellation-like behavior, because changing a reservation can release old nights and reserve new ones.

Manage Listing Availability remains separate. A host can block nights independently of reservations. Cancelling a reservation does not remove host blocks.

Manage Booking Eligibility remains separate. A guest may be blocked from making new reservations, but that does not automatically decide whether an existing reservation can be canceled.

## Capability summary

The Cancel a Reservation capability cancels an existing confirmed reservation only when the current reservation situation allows it.

It starts with the intent to cancel. It evaluates the reservation, the requester, the current reservation status, and the cancellation timing. If the cancellation is valid, the reservation is canceled and no longer occupies the listing’s nights. If not, the cancellation is rejected with a clear business reason.

The capability is small, but it is important. It shows that the booking situation changes not only when new reservations are confirmed, but also when existing reservations stop being active.
