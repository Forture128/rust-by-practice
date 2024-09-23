# Core System Behind Tinder - Requirement Document

## Overview

The core system behind Tinder is a dating application that allows users to create a profile and interact with a deck of potential matches through swiping. This document outlines the high-level requirements for building the core features of the application.

## Functional Requirements

1. User Profile Management
   - Users should be able to create a profile with basic information, including name, age, bio, and photos.
   - Users can view and edit their profile details.

2. Swiping Functionality
   - Users can swipe right to like a potential match or left to dislike them.
   - Users can super-like a potential match to stand out.
   - Users can undo their most recent swipe if it was a left swipe.

3. Matching Algorithm
   - The system should match users based on their preferences and location.
   - Matches are made when two users swipe right on each other.

4. User Engagement
   - The system should not have any limitations on the number of swipes, Super Likes, and Undos that users can perform per day.

## Non-Functional Requirements

1. Performance
   - The application should provide mostly instant swipes to ensure a smooth user experience.
   - Latency is acceptable during app startup and after a user has swiped through a considerable number of potential matches.

2. Scalability
   - The system should support a global user base of approximately 50 million users, evenly distributed across the world.

3. Redundancy
   - The system should handle failures without significant impact on user experience or data integrity.

4. Security
   - User data should be encrypted at rest and in transit to ensure privacy and data security.
   - Measures should be in place to prevent unauthorized access to user accounts.

5. User Notifications
   - Users should receive real-time notifications when they get a match, directly after swiping right on a potential match.

## Future Enhancements

1. Real-Time Chat
   - Implement real-time messaging between matched users.

2. User Preferences
   - Allow users to set preferences for age range, distance, and other criteria.

3. Reporting and Moderation
   - Implement a reporting and moderation system to handle inappropriate content and behavior.

4. User Verification
   - Provide optional user verification mechanisms to enhance trust and authenticity.

## Constraints

1. The focus of this project is on the core system, and advanced features like real-time chat and user verification are considered as future enhancements.

2. The system does not include a notification system for matches that occur outside the act of swiping right on a potential match.

## Acceptance Criteria

- Users can create and manage their profiles with required information.
- Swiping functionality allows users to swipe right, left, and super-like potential matches.
- Undo functionality allows users to undo their most recent left swipe.
- Users can receive real-time notifications when they get a match.

## Glossary

- Swipe: The action of a user swiping left, right, or super-like on a potential match.
- Super Like: A special swipe action that puts the user at the top of the other users' decks.
