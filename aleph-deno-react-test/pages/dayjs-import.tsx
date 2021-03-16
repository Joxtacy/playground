import React from "react";
import dayjs from "https://esm.sh/dayjs";

export default function DayjsImport() {
    return <section>Hello DayJS! Today is: {dayjs().format("DD-MM-YYYY")}</section>;
}