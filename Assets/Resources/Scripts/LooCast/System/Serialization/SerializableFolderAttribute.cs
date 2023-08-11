﻿using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class SerializableFolderAttribute : Attribute
    {
        #region Properties
        public bool OverrideSerialization { get; private set; }
        public bool OverrideDeserialization { get; private set; }
        #endregion

        #region Constructors
        public SerializableFolderAttribute(bool overrideSerialization = false, bool overrideDeserialization = false)
        {
            OverrideSerialization = overrideSerialization;
            OverrideDeserialization = overrideDeserialization;
        }
        #endregion
    }
}
