interface Props {
  children?: React.ReactNode;
}

export function RouterProvider({ children }: Props) {
  return (
    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      {children || (
        <div
          style={{
            flex: 1,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexDirection: 'column',
            gap: '1rem',
          }}
        >
          <h1>FHIR Profile Builder</h1>
          <p>React App with FSD Architecture</p>
          <p style={{ color: '#868e96', fontSize: '0.875rem' }}>
            ✓ Vite + React + TypeScript
            <br />✓ Effector State Management
            <br />✓ TanStack Query
            <br />✓ Mantine UI
            <br />✓ Feature-Sliced Design
          </p>
        </div>
      )}
    </div>
  );
}
