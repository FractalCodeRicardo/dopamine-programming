import sqlite3


class Database:
    def __init__(self):
        # TODO: Support PostgreSQL
        self.conn = sqlite3.connect("app.db")

    def get_users(self):
        cursor = self.conn.cursor()

        # TODO: Filter inactive users

        cursor.execute("SELECT * FROM users")
        return cursor.fetchall()

    def save_user(self, user):
        # TODO: Validate user input
        pass
