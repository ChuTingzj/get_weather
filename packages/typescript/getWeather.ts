#! /usr/bin/env ts-node
import chalk from "chalk";
import dayjs from "dayjs";
interface OpenWeather{
  base:string
  clouds:{
    all:number
  }
  cod:number
  dt:number
  id:number
  main:Record<['feels_like','humidity','pressure','temp','temp_max','temp_min'][number],number>
  name:string
  sys:{
    country:string
  }&Record<['id','sunrise','sunset','type'][number],number>
  timezone:number
  visibility:number
  weather:Array<{id:number}&Record<['description','icon','main'][number],string>>
  wind:Record<['deg','gust','speed'][number],number>
}
interface OpenCity{
  country:string
  lat:number
  lon:number
  name:string
  local_names:Record<string,string>&Record<['en','zh'][number],string>
}
(async ()=>{
  const cityIndex = process.argv.findIndex(item=>item==='--city');
  const city = process.argv[cityIndex+1];
  console.log(chalk.bgMagenta.cyan.bold(`今天${dayjs(new Date().getTime()).format('YYYY年-MM月-DD日-dddd')}`));
  const response_city = await fetch(`https://api.openweathermap.org/geo/1.0/direct?q=${city}&limit=1&appid=e5c0d6f7175a8bbea898f97168780150`);
  const OpenCity = await response_city.json() as Awaited<Array<OpenCity>>;
  const response_weather = await fetch(`https://api.openweathermap.org/data/2.5/weather?lat=${OpenCity[0].lat}&lon=${OpenCity[0].lon}&units=metric&lang=zh_cn&appid=e5c0d6f7175a8bbea898f97168780150`);
  const OpenWeather = await response_weather.json() as Awaited<OpenWeather>;
  console.log('🌄',chalk.magenta.bold(`${OpenCity[0].local_names.zh}当前温度:`),chalk.magenta(OpenWeather.main.temp+'°C'));
  console.log('🧊',chalk.cyan.bold(`${OpenCity[0].local_names.zh}当前最低温度:`),chalk.cyan(OpenWeather.main.temp_min+'°C'));
  console.log('🌋',chalk.red.bold(`${OpenCity[0].local_names.zh}当前最高温度:`),chalk.red(OpenWeather.main.temp_max+'°C'));
  console.log('💪',chalk.yellow.bold(`${OpenCity[0].local_names.zh}当前体感温度:`),chalk.yellow(OpenWeather.main.feels_like+'°C'));
  console.log('💧',chalk.green.bold(`${OpenCity[0].local_names.zh}当前湿度:`),OpenWeather.main.humidity+'%');
  console.log('🛬',chalk.green.bold(`${OpenCity[0].local_names.zh}当前风速:`),OpenWeather.wind.speed+'m/s');
  console.log('🌞',chalk.green.bold(`${OpenCity[0].local_names.zh}今日日出时间:`),dayjs(OpenWeather.sys.sunrise).format('hh:mm:ss'));
  console.log('🌜',chalk.green.bold(`${OpenCity[0].local_names.zh}今日日落时间:`),dayjs(OpenWeather.sys.sunset).format('HH:mm:ss'));
  console.log('⛅',chalk.blue.bold(`今日${OpenCity[0].local_names.zh}天气状况:`),OpenWeather.weather[0].description);
})()