# Welcome to your Lovable project

## Project info

**URL**: https://lovable.dev/projects/REPLACE_WITH_PROJECT_ID

## How can I edit this code?

There are several ways of editing your application.

**Use Lovable**

Simply visit the [Lovable Project](https://lovable.dev/projects/REPLACE_WITH_PROJECT_ID) and start prompting.

Changes made via Lovable will be committed automatically to this repo.

**Use your preferred IDE**

If you want to work locally using your own IDE, you can clone this repo and push changes. Pushed changes will also be reflected in Lovable.

The only requirement is having Node.js & npm installed - [install with nvm](https://github.com/nvm-sh/nvm#installing-and-updating)

Follow these steps:

```sh
# Step 1: Clone the repository using the project's Git URL.
git clone <YOUR_GIT_URL>

# Step 2: Navigate to the project directory.
cd <YOUR_PROJECT_NAME>

# Step 3: Install the necessary dependencies.
npm i

# Step 4: Configure environment variables (optional)
cp env.example .env
# Edit .env to configure API endpoints and demo wallet

# Step 5: Start the development server with auto-reloading and an instant preview.
npm run dev
```

## Connecting to DefPool Server

The portal connects to the DefPool server API for real-time data. Make sure the DefPool server is running on `http://localhost:3000` or configure a different URL:

```env
VITE_API_BASE_URL=http://localhost:3000
```

### API Endpoints Used

- `GET /api/v1/targets` - Profitability scores for all mining targets
- `GET /api/v1/target` - Current mining target
- `GET /api/v1/targets/current` - Current target name
- `GET /api/v1/miners/{wallet}/workers` - Worker statistics
- `GET /api/v1/miners/{wallet}/stats` - Miner statistics

### Demo Wallet

For testing, a demo wallet is configured. In production, implement proper wallet selection:

```env
VITE_DEMO_WALLET=44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT
```

**Edit a file directly in GitHub**

- Navigate to the desired file(s).
- Click the "Edit" button (pencil icon) at the top right of the file view.
- Make your changes and commit the changes.

**Use GitHub Codespaces**

- Navigate to the main page of your repository.
- Click on the "Code" button (green button) near the top right.
- Select the "Codespaces" tab.
- Click on "New codespace" to launch a new Codespace environment.
- Edit files directly within the Codespace and commit and push your changes once you're done.

## What technologies are used for this project?

This project is built with:

- Vite
- TypeScript
- React
- shadcn-ui
- Tailwind CSS

## How can I deploy this project?

Simply open [Lovable](https://lovable.dev/projects/REPLACE_WITH_PROJECT_ID) and click on Share -> Publish.

## Can I connect a custom domain to my Lovable project?

Yes, you can!

To connect a domain, navigate to Project > Settings > Domains and click Connect Domain.

Read more here: [Setting up a custom domain](https://docs.lovable.dev/features/custom-domain#custom-domain)
