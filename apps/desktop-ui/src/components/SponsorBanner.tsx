type SponsorBannerProps = {
  text: string;
  note: string;
};

export function SponsorBanner({ text, note }: SponsorBannerProps) {
  return (
    <section className="sponsor-banner" aria-label="Sponsor placeholder">
      <strong>{text}</strong>
      <span>{note}</span>
    </section>
  );
}
