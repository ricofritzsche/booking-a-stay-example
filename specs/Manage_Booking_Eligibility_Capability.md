# Manage Booking Eligibility Capability

The **Manage Booking Eligibility** capability covers the moment when the platform changes whether a guest may book stays or whether a listing may receive reservations.

A guest can be blocked from making new reservations. A blocked guest may still have existing reservations, but they cannot confirm new ones. A listing can also be disabled from receiving new bookings. A disabled listing may still exist, but guests cannot book new stays for it.

The capability is not about editing a guest profile or changing a listing description. It is about changing whether future reservations are allowed.

## Domain language

A **guest** is the person who wants to book stays.

An **eligible guest** is a guest who is currently allowed to make new reservations.

A **blocked guest** is a guest who is not allowed to make new reservations.

A **listing** is the place offered for booking.

A **bookable listing** is a listing that can currently receive reservations.

A **disabled listing** is a listing that cannot currently receive new reservations.

A **booking eligibility change** is a decision to block or restore a guest’s ability to book, or to disable or restore a listing’s ability to receive reservations.

## Business purpose

The purpose of this capability is to control whether future booking decisions are allowed at all.

Availability alone is not enough to confirm a reservation. A listing may have free nights, but if the listing is disabled, no new reservation should be confirmed. A guest may request valid dates and a valid guest count, but if the guest is blocked, the booking should be rejected.

The capability answers one business question:

> Should this guest or listing be allowed to participate in future bookings?

If the answer changes, the booking situation changes for future reservation decisions.

## Eligibility request

A booking eligibility request contains the information needed to evaluate the change:

```text
target type: guest or listing
target
requested change
changed by
change time
reason
```

The requested change can be:

```text
block guest
restore guest eligibility
disable listing
restore listing bookability
```

The request expresses an administrative or operational intent. It does not automatically mean the change can be applied. The system still has to evaluate whether the guest or listing exists, whether the requester is allowed to make the change, and whether the requested change makes sense for the current status.

## What the capability has to evaluate

To decide whether booking eligibility can be changed, the capability needs to evaluate the current situation of the guest or listing.

For a guest, it must know whether the guest exists and whether the guest is currently eligible or blocked. Blocking an already blocked guest may be rejected or treated as already satisfied, depending on the product decision. Restoring eligibility for a guest who is already eligible follows the same rule.

For a listing, it must know whether the listing exists and whether the listing is currently bookable or disabled. Disabling an already disabled listing may be rejected or treated as already satisfied. Restoring a listing that is already bookable may also be treated as no change.

The capability must also know whether the requester is allowed to make the change. In the first version, this can be a platform action. Later, some listing-level changes may be allowed for hosts, while guest-level blocks may remain platform-only.

## Guest eligibility conditions

A guest can be blocked only when the required conditions are true:

```text
the guest exists
the requester is allowed to block the guest
the guest is currently eligible to book
```

A blocked guest can be restored only when the required conditions are true:

```text
the guest exists
the requester is allowed to restore the guest
the guest is currently blocked
```

The important point is that guest eligibility controls future reservations. It does not automatically cancel existing reservations. Whether existing reservations should be cancelled is a separate business decision.

## Listing bookability conditions

A listing can be disabled only when the required conditions are true:

```text
the listing exists
the requester is allowed to disable the listing
the listing is currently bookable
```

A disabled listing can be restored only when the required conditions are true:

```text
the listing exists
the requester is allowed to restore the listing
the listing is currently disabled
```

Disabling a listing prevents new reservations. It does not remove existing reservations and it does not replace host availability management. A listing can be bookable in general and still have individual nights blocked. A listing can also have open nights but be disabled completely.

## Rejection reasons

A booking eligibility change should be rejected with a clear business reason.

Typical rejection reasons are:

```text
guest not found
listing not found
requester is not allowed to make this change
guest is already blocked
guest is already eligible
listing is already disabled
listing is already bookable
change reason is required
```

The rejection reason belongs to the domain. It should explain why the eligibility change was not accepted, not expose how the system stores guest or listing status.

## Effect on the booking situation

Managing booking eligibility directly affects future booking decisions.

If a guest is blocked, **Book a Stay** must reject new booking requests from this guest, even when the listing and requested nights are otherwise available.

If a guest’s eligibility is restored, the guest may book again, as long as the listing, requested dates, guest count, and booking rules allow it.

If a listing is disabled, **Book a Stay** must reject new booking requests for this listing, even when the requested nights are free.

If a listing is restored, it can receive new reservations again, as long as its nights are available and its booking rules are satisfied.

This keeps the meaning clear: booking eligibility does not decide whether a specific night is free. It decides whether the guest or listing may be used in a new reservation decision.

## Successful outcome

When the eligibility change is accepted, the system records the changed booking situation.

For a guest, the result is:

```text
guest
new booking eligibility
changed by
change time
reason
```

For a listing, the result is:

```text
listing
new booking status
changed by
change time
reason
```

From the domain perspective, the important result is that future booking decisions now use the changed eligibility or bookability.

## Business examples

The platform blocks a guest after a trust or policy decision. The guest exists and is currently eligible to book. The system blocks the guest from making new reservations.

The blocked guest tries to book a stay for dates that are otherwise available. The system rejects the booking because the guest is blocked.

The platform restores the guest’s ability to book. The guest is currently blocked. The system marks the guest as eligible again.

The platform disables a listing because it should not receive new reservations. The listing exists and is currently bookable. The system disables the listing.

A guest tries to book a disabled listing. The requested nights are free, but the listing cannot receive bookings. The system rejects the booking because the listing is disabled.

The platform restores a disabled listing. The listing can receive new reservations again, but only when the requested nights are available and the booking rules are satisfied.

## What this capability does not cover

This capability manages whether guests and listings may participate in future bookings. It does not manage every detail of a guest or listing.

Changing a guest’s name, email address, profile photo, or contact details belongs elsewhere.

Changing a listing title, description, photos, amenities, address, pricing, minimum stay, maximum stay, or guest capacity belongs elsewhere.

Cancelling existing reservations is also outside this capability. Blocking a guest or disabling a listing may lead to later operational decisions, but those decisions should be handled by separate capabilities.

## Relation to other capabilities

Manage Booking Eligibility directly influences **Book a Stay**. A blocked guest cannot confirm a new reservation. A disabled listing cannot receive a new reservation.

It also influences **Change a Reservation** if the product rules require changed reservations to respect current guest eligibility or listing bookability.

It remains separate from **Cancel a Reservation**. Cancelling ends an existing reservation. Blocking a guest or disabling a listing controls future booking decisions.

It remains separate from **Manage Listing Availability**. Availability controls specific nights. Booking eligibility controls whether the guest or listing is allowed to participate in bookings at all.

## Capability summary

The Manage Booking Eligibility capability changes whether a guest may book stays or whether a listing may receive reservations.

It starts with the intent to block, restore, disable, or restore bookability. It evaluates the target, the requester, the current status, and the reason for the change. If the change is valid, future booking decisions use the new eligibility or bookability. If not, the request is rejected with a clear business reason.

The capability is important because a reservation is not confirmed by availability alone. A valid booking decision depends on the guest, the listing, and the requested nights all being valid for booking.
