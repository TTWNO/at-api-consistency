# Formal Specification of an ACI-Compliant AT API

## Abstract

The Synchronous Assistive Technology API (SATAPI?) is a stateful application-level interface for communicating UI state to assistive technologies.
This document describes the overall architecture of SATAPI?, defines nomenclature, and TODO.

## Status

TODO

## Copyright Notice

This document is copyright by Tait Hoyem (TODO: add other contributors) ... and is available under the CC-0 International license.

## 1. Introduction

### 1.1 Purpose

The SATAPI is a stateful, application level event structuring and RPC ordering protocol that contains precise semantics.

### 1.2 History

SATAPI draws on the experience of assistive technology (AT) developers in relation to the platform APIs currently in production.
It started as a part of the UI event loop, with accessibility events pushed as part of the event loop of an application.
Then, write commands were able to be executed by requesting changes from the client to the server.

This protocol (SATAPI) is designed to resolve determinism issues with the previous APIs defined on a platform-by-platform basis.

### 1.3 Core Semantics

SATAPI defines a common interface for defining and manipulating UI state.

Each message is either a) an incremental tree update, b) a request, or c) a response.
A client (AT) constructs an accessibility tree by receiving and applying counter-timestamped incremental tree updates over time.
A server (application) listens for requests and transmits incremental tree updates upon each update of its projected view into an AT.
The server responds with an counter-timestamped response, synchronous with the incremental updates such that responses always occur between two specific tree updates.



