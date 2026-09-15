import { CATEGORY_ICONS, CATEGORY_NAMES, CHECKS_PER_CATEGORY, REPORT_OPTIONS, SECTIONS } from './data';
import { ReportOption, ScoreType } from './types';

describe('data', () => {
  it('defines each non-global score section once across section metadata and category maps', () => {
    const expectedSections = Object.values(ScoreType).filter((scoreType) => scoreType !== ScoreType.Global);
    const sectionTypes = SECTIONS.map((section) => section.type);

    expect(sectionTypes).toHaveLength(expectedSections.length);
    expectedSections.forEach((scoreType) => {
      expect(sectionTypes.filter((sectionType) => sectionType === scoreType)).toHaveLength(1);
      expect(CATEGORY_NAMES).toHaveProperty(scoreType);
      expect(CATEGORY_ICONS).toHaveProperty(scoreType);
    });
  });

  it('defines metadata and one category for every report option', () => {
    const checks = Object.values(CHECKS_PER_CATEGORY).flat();

    Object.values(ReportOption).forEach((option) => {
      expect(REPORT_OPTIONS).toHaveProperty(option);
      expect(REPORT_OPTIONS[option].reference).toBeTruthy();
      expect(checks.filter((check) => check === option)).toHaveLength(1);
    });
  });

  it('keeps compact report option names within the rendered width budget', () => {
    Object.values(REPORT_OPTIONS).forEach((option) => {
      expect((option.shortName || option.name).length).toBeLessThanOrEqual(22);
    });
  });

  it('marks only Agent Readiness as advisory', () => {
    expect(SECTIONS.filter((section) => section.advisory).map((section) => section.type)).toEqual([
      ScoreType.AgentReadiness,
    ]);
  });

  it('lists Agent Readiness checks alphabetically by display name', () => {
    expect(CHECKS_PER_CATEGORY[ScoreType.AgentReadiness]).toEqual([
      ReportOption.Authentication,
      ReportOption.ContentDiscoverability,
      ReportOption.ContentStructure,
      ReportOption.MarkdownAvailability,
      ReportOption.Observability,
      ReportOption.PageSize,
      ReportOption.UrlStability,
    ]);
  });
});
