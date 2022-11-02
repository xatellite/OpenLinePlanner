export const secondsToMinSecPadded = (time) => {
  const minutes = Math.floor(time / 60);
  let minutesString = minutes;
  if (minutes < 10) {
    minutesString = "0" + minutes;
  }
  const seconds = "0" + Math.round(time - (minutes * 60));
  return minutesString + ":" + seconds.substr(-2);
};
