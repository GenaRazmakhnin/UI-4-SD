import { Button, Code, Container, Stack, Text, Title } from '@mantine/core';
import { IconAlertTriangle } from '@tabler/icons-react';
import { Component, type ErrorInfo, type ReactNode } from 'react';
import styles from './ErrorBoundary.module.css';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
    this.setState({ errorInfo });

    // Log to error reporting service
    // reportError(error, errorInfo);
  }

  handleReset = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
    window.location.href = '/';
  };

  render() {
    if (this.state.hasError) {
      return (
        <Container size="sm" className={styles.container}>
          <Stack align="center" gap="lg">
            <IconAlertTriangle size={64} color="var(--error-color)" />

            <Title order={2}>Something went wrong</Title>

            <Text size="sm" c="dimmed" ta="center">
              An unexpected error occurred. Please try refreshing the page or returning to the home
              page.
            </Text>

            {this.state.error && (
              <Code block className={styles.errorCode}>
                {this.state.error.toString()}
                {this.state.errorInfo && (
                  <>
                    {'\n\n'}
                    {this.state.errorInfo.componentStack}
                  </>
                )}
              </Code>
            )}

            <Stack gap="sm" w="100%">
              <Button onClick={this.handleReset} fullWidth>
                Return to Home
              </Button>
              <Button variant="subtle" onClick={() => window.location.reload()} fullWidth>
                Refresh Page
              </Button>
            </Stack>
          </Stack>
        </Container>
      );
    }

    return this.props.children;
  }
}
