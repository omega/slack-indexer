# WARNING: Might break Slack API Terms of Service

**This application stores messages that matches patterns, so those messages can
be retrieved at a later point. This might break the "6.1 No Storing" clause of
the Slack API Terms of Service. You use it at your own risk.

# Purpose

The purpose of this web app is to point an outgoing webhook from Slack to an
API endpoint for all or some channels in your slack. Then this app will look
for configured patterns in those messages, and if those patterns match, it will
store a reference to that message in it's database.

There is then a second endpoint in the API, that will take a string as an
argument, and then find all messages where one of the patterns matched that
string.

# Example

We use JIRA at our company, so we setup a pattern to match JIRA issue keys
([A-Z]+-\d+). A string ABC-123 would match this pattern.

If we then get the following message from slack as an outgoing webhook:

> Hey, do you remember what we were thinking about ABC-123?

That message would then match our pattern, and we would store a reference to
that message in our database, along with the string ABC-123 as the "matched
string".

Later we could then make a request to the search endpoint with the string
"ABC-123" as our search. We would then get back this message, with a URL that
points to the slack archives for that message.

There could then be a partner JIRA plugin that would perform these requests,
and show the results in the JIRA interface for the issue ABC-123, and allow to
jump to discussions made about the issue easily.
