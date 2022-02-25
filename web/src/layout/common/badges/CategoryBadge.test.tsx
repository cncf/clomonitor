import { render, screen } from '@testing-library/react';

import { Category } from '../../../types';
import CategoryBadge from './CategoryBadge';

describe('CategoryBadge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<CategoryBadge categoryId={Category['App definition']} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders category 0', () => {
    render(<CategoryBadge categoryId={Category['App definition']} />);
    expect(screen.getByText('App definition')).toBeInTheDocument();
  });

  it('renders category 1', () => {
    render(<CategoryBadge categoryId={Category.Observability} />);
    expect(screen.getByText('Observability')).toBeInTheDocument();
  });

  it('renders category 2', () => {
    render(<CategoryBadge categoryId={Category.Orchestration} />);
    expect(screen.getByText('Orchestration')).toBeInTheDocument();
  });

  it('renders category 3', () => {
    render(<CategoryBadge categoryId={Category.Platform} />);
    expect(screen.getByText('Platform')).toBeInTheDocument();
  });

  it('renders category 4', () => {
    render(<CategoryBadge categoryId={Category.Provisioning} />);
    expect(screen.getByText('Provisioning')).toBeInTheDocument();
  });

  it('renders category 5', () => {
    render(<CategoryBadge categoryId={Category.Runtime} />);
    expect(screen.getByText('Runtime')).toBeInTheDocument();
  });

  it('renders category 6', () => {
    render(<CategoryBadge categoryId={Category.Serverless} />);
    expect(screen.getByText('Serverless')).toBeInTheDocument();
  });
});
