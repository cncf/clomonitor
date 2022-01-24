import { isNull } from 'lodash';

import getMetaTag from './getMetaTag';

const tinycolor = require('tinycolor2');

interface ColorItem {
  name: string;
  color: string;
}

class ThemeBuilder {
  private primary: string | null = null;
  private secondary: string | null = null;
  private customColors: ColorItem[] = [];
  private sheet: CSSStyleSheet | null = null;

  public init() {
    this.primary = getMetaTag('primaryColor');
    this.secondary = getMetaTag('secondaryColor');
    this.createStyleSheet();
    this.prepareCustomColors();
    this.applyColors();
  }

  private createStyleSheet() {
    const style = document.createElement('style');
    style.appendChild(document.createTextNode(''));
    document.head.appendChild(style);
    this.sheet = style.sheet;
  }

  private prepareCustomColors() {
    if (!isNull(this.primary) && tinycolor(this.primary).isValid()) {
      this.customColors.push({ name: '--rm-primary', color: this.primary });
      this.customColors.push({ name: '--bs-primary', color: this.primary });
      const rgbColor = tinycolor(this.primary).toRgb();
      this.customColors.push({ name: '--bs-primary-rgb', color: `${rgbColor.r}, ${rgbColor.g}, ${rgbColor.b}` });
      this.customColors.push({ name: '--rm-primary-50', color: tinycolor(this.primary).setAlpha(0.5).toRgbString() });
      this.customColors.push({ name: '--rm-primary-5', color: tinycolor(this.primary).setAlpha(0.05).toRgbString() });
    }

    if (!isNull(this.secondary) && tinycolor(this.secondary).isValid()) {
      this.customColors.push({ name: '--rm-secondary', color: this.secondary });
      this.customColors.push({ name: '--bs-secondary', color: this.secondary });
      this.customColors.push({ name: '--rm-secondary-900', color: tinycolor(this.secondary).darken(10).toHexString() });
      const rgbColor = tinycolor(this.secondary).toRgb();
      this.customColors.push({ name: '--bs-secondary-rgb', color: `${rgbColor.r}, ${rgbColor.g}, ${rgbColor.b}` });
      this.customColors.push({
        name: '--rm-secondary-50',
        color: tinycolor(this.secondary).setAlpha(0.5).toRgbString(),
      });
      this.customColors.push({
        name: '--rm-secondary-15',
        color: tinycolor(this.secondary).setAlpha(0.15).toRgbString(),
      });
      this.customColors.push({ name: '--highlighted', color: this.secondary });
    }
  }

  public applyColors() {
    if (!isNull(this.sheet)) {
      const colorsList = this.customColors.map((item: ColorItem) => `${item.name}: ${item.color};`);
      this.sheet.insertRule(`[data-theme='light'] { ${colorsList.join('')} }`, 0);
    }
  }
}

const themeBuilder = new ThemeBuilder();
export default themeBuilder;
