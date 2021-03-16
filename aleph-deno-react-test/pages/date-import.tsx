import React from "react";
import { format } from "https://deno.land/std@0.88.0/datetime/mod.ts";

export default function DateImport() {
    return <section>Hello DateImport! Today is: {format(new Date(), "dd-MM-yyyy")}</section>;
}