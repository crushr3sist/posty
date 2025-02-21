"use client";

export default function ErrorMessage({ error, reset }) {
  return (
    <div>
      <h1>something went wrong</h1>
      <p>{error.message}</p>
    </div>
  );
}
