interface Props {
  children: React.ReactNode;
}

/**
 * For client-side SPAs, we use Effector stores globally without fork().
 * fork() is mainly useful for SSR or testing with isolated scopes.
 */
export function EffectorProvider({ children }: Props) {
  return <>{children}</>;
}
