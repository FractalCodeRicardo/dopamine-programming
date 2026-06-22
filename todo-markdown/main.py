from database import Database
from email_service import EmailService

# TODO: Load configuration from environment variables


def main():
    db = Database()
    emails = EmailService()

    users = db.get_users()

    # TODO: Add pagination for large datasets

    for user in users:
        emails.send_welcome(user)

    # TODO: Add error reporting


if __name__ == "__main__":
    main()
