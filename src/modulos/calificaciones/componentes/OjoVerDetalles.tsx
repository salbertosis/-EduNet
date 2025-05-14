import React from 'react';

interface OjoVerDetallesProps {
  onClick: () => void;
  title?: string;
}

export function OjoVerDetalles({ onClick, title = 'Ver detalles' }: OjoVerDetallesProps) {
  return (
    <button
      onClick={onClick}
      className="rounded-full p-2 text-primary-500 hover:text-white hover:bg-primary-600 hover:shadow-md hover:shadow-primary-400/30 transition-all"
      title={title}
      aria-label={title}
      type="button"
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5">
        <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12s3.75-6.75 9.75-6.75S21.75 12 21.75 12s-3.75 6.75-9.75 6.75S2.25 12 2.25 12z" />
        <circle cx="12" cy="12" r="3" />
      </svg>
    </button>
  );
} 