type PlaceholderScreenProps = {
  title: string;
  description: string;
  badge: string;
};

export function PlaceholderScreen({ title, description, badge }: PlaceholderScreenProps) {
  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{badge}</p>
        <h1>{title}</h1>
        <p>{description}</p>
      </div>
      <div className="empty-state">
        <div className="empty-state-grid">
          <span />
          <span />
          <span />
          <span />
        </div>
      </div>
    </section>
  );
}
