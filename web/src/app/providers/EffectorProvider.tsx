import { fork } from 'effector';
import { Provider } from 'effector-react';

interface Props {
  children: React.ReactNode;
}

const scope = fork();

export function EffectorProvider({ children }: Props) {
  return <Provider value={scope}>{children}</Provider>;
}
