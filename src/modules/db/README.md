# Database Module

The database module contains solutions for gathering database information, analyzing databases in real-time, and consistent database communication.

The following databases are supported:

|  Database  |  Location                    |
|------------|------------------------------|
|  Redis     |  db/redis/redis_analysis.py  |

NOTE: These are unique modules that spawn individual shells to communicate with the database. In most cases, Catherine will still be active, so once you exit the database shell, it'll drop you back into the Catherine prompt.