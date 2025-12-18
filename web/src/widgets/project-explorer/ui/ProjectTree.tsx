import * as ReactWindow from 'react-window';
import { List } from 'react-window';
import { ProjectTreeRow, type ProjectTreeRowData } from './ProjectTreeRow';
import styles from './ProjectExplorer.module.css';

interface ProjectTreeProps extends ProjectTreeRowData {}

export function ProjectTree(props: ProjectTreeProps) {

  return (
    <div className={styles.treeList} role="tree" aria-label="Project files">
      <List
        defaultHeight={520}
        rowCount={props.rows.length}
        rowHeight={40}
        rowComponent={ProjectTreeRow}
        rowProps={props}
        style={{ height: '100%', width: '100%' }}
      />
    </div>
  );
}
