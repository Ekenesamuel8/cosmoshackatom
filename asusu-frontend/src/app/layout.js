export default function Layout({ children }) {
  return (
      <html lang="en">
          <body className="bg-gray-100 text-gray-900">
              <nav className="bg-blue-600 p-4 text-white">
                  <h1 className="text-xl font-bold">Asusu Contribution WebApp</h1>
              </nav>
              <main className="container mx-auto mt-6">{children}</main>
          </body>
      </html>
  );
}
