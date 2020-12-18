import {
  LoggerFactory,
  LoggerFactoryOptions,
  LFService,
  LogGroupRule,
  LogLevel,
  getLogControl,
  LogGroupControlSettings,
} from 'typescript-logging';

const options = new LoggerFactoryOptions()
  .addLogGroupRule(new LogGroupRule(new RegExp('.+'), LogLevel.Debug));

export const formatFilename = (name) => {
  const t = name.split('/');
  return t[t.length - 1];
};

export const factory = LFService.createNamedLoggerFactory('StateTester', options);

const control = getLogControl();

// Factories are numbered, use listFactories() to find out
export const factoryControl = control.getLoggerFactoryControl(0);
