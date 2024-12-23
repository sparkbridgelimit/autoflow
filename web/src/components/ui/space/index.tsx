import React, { ReactNode } from 'react';
import './index.css'; // 引入样式文件

interface SpaceProps {
  direction?: 'horizontal' | 'vertical';
  size?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
  children: ReactNode;
}

const Space: React.FC<SpaceProps> = ({ direction = 'horizontal', size = 2, children }) => {
  const className = `space space-${direction} space-size-${size}`;

  return (
    <div className={className}>
      {React.Children.map(children, (child, index) => (
        <div key={index} className="space-item">
          {child}
        </div>
      ))}
    </div>
  );
};

export default Space;