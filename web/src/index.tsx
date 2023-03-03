import './index.css';

import { createRoot } from 'react-dom/client';

import App from './App';

const container = document.getElementById('clo-wrapper');
const root = createRoot(container!);

root.render(<App />);
