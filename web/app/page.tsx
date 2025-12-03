export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm">
        <h1 className="text-4xl font-bold text-center mb-8">
          Noether - Perpetual DEX
        </h1>
        <p className="text-center text-lg mb-4">
          GMX-style Perpetual DEX on Stellar Futurenet
        </p>
        <div className="mt-8 text-center">
          <p className="text-gray-500">
            Connect your wallet to get started
          </p>
        </div>
      </div>
    </main>
  )
}


