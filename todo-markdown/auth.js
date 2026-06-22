export async function login(username, password) {
  // TODO: Add rate limiting

  const response = await fetch("/api/login", {
    method: "POST",
    body: JSON.stringify({ username, password })
  });

  // TODO: Handle network failures

  return response.json();
}

export function logout() {
  // TODO: Clear cached user data
}
