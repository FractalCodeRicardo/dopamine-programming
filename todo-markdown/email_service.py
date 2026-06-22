class EmailService:
    def send_welcome(self, user):
        # TODO: Use HTML templates

        subject = "Welcome"
        body = f"Hello {user['name']}"

        # TODO: Add retry mechanism

        print(subject)
        print(body)

    def send_newsletter(self):
        # TODO: Implement newsletter sending
        pass
