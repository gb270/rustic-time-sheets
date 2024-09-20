## Feature roadmap

- ~add functionality to enable custom project names~
- add functionality to enable custom price codes

- implement history tab so that past activities can be viewed from the app
 (can maybe reuse the timesheet.csv then as a dependency? i.e. that is the file that we read
 from for persistence?)
- history tab should also show more info about price codes (e.g. what the price code represents)
- implement time zones, at the moment all recorded times just remain in UTC
- implement settings tabs so that project names and price codes can be edited and deleted

- clean up console prints in order to release V1.0!
- might need to add functionality to run multiple tasks at once?

## Bug fixes

- Additional dialog boxes that were created stay open even if main window is closed.
- - In progress!
- entering custom project name requires pressing enter and then ok. (Should just require pressing ok)