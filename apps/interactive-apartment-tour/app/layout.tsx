import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Koya Apartment 106 | Interactive Concept Tour',
  description: 'Explore the Apartment 106 floor plan through an interactive, street-view-inspired concept tour.',
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}
