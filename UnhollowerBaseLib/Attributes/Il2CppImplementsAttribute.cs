﻿using System;

namespace UnhollowerBaseLib.Attributes
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class Il2CppImplementsAttribute : Attribute
    {
        public Type[] Interfaces { get; private init; }

        public Il2CppImplementsAttribute(params Type[] interfaces)
        {
            Interfaces = interfaces;
        }
    }
}
