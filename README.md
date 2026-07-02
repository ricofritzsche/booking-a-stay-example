# Booking a Stay Example

This repository contains a Rust example application for a stay booking domain.

The example uses familiar booking language: guests book stays, hosts offer listings, and the system confirms reservations. It is not a clone of Airbnb or Booking.com. The goal is to model the core booking behavior in a way that can grow without introducing a central domain model too early.

A guest chooses a listing, a check-in date, a check-out date, and a guest count. The system confirms a reservation only when the current booking situation allows it.

That means:

```text
the guest is allowed to book
the listing can receive bookings
the date range is valid
the guest count fits the listing rules
the requested nights are available
no confirmed reservation or host block prevents the stay
```

The important business result is a confirmed reservation, not a generic database update.

## Purpose

This repository demonstrates how to build a ready-to-grow application around Domain Capabilities.

The Application State is stored in PostgreSQL tables. The example does not use an event store. It still keeps the command flow explicit: a request enters a capability, the relevant state is loaded, the decision is made, and the result is recorded only when the decision context is still valid.

The main technical ideas are documented separately. The README only gives the orientation.

## Domain Capabilities

The domain is described through these capabilities:

```text
Book a Stay
Cancel a Reservation
Change a Reservation
Manage Listing Availability
Manage Booking Eligibility
```

The first implementation focuses on **Book a Stay**. The other capabilities describe how the application can grow while keeping business behavior separated by intent.

## Documents

Start with the domain overview:

- [Domain overview](DOMAIN.md)

Capability specifications:

- [Book a Stay](specs/Book_a_Stay_Capability.md)
- [Cancel a Reservation](specs/Cancel_a_Reservation_Capability.md)
- [Change a Reservation](specs/Change_a_Reservation_Capability.md)
- [Manage Listing Availability](specs/Manage_Listing_Availability_Capability.md)
- [Manage Booking Eligibility](specs/Manage_Booking_Eligibility_Capability.md)

Implementation guidance:

- [Guidelines index](guidelines/00_index.md)

The domain documents describe the business behavior. The guidelines describe how the code should apply Command Context Consistency, Functional Core and Imperative Shell, Request Processing Units, and relational Application State.

## Core Idea

The repository separates business behavior from storage structure.

Tables store Application State. Capabilities interpret the data they need for their own decisions. A capability may use several tables, and several capabilities may use the same tables, but the meaning of the data stays close to the capability that uses it.

For the booking flow, the central question is:

```text
Can this guest book this listing for this date range with this number of guests?
```

If the answer is yes, the system confirms a reservation. If not, the booking is rejected with a clear business reason.

## Scope

The example intentionally stays focused.

It does not cover payments, pricing, taxes, refunds, host approval workflows, reviews, guest messaging, fraud checks, or cancellation policies in detail. These concerns can become separate capabilities later.

The focus is the booking decision and the consistency boundary around that decision.
