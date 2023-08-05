using System;

namespace LooCast.System.Serialization
{
    public abstract class SerializableAttribute : Attribute
    {
        #region Properties
        public bool OverrideSerialization { get; private set; }
        public bool OverrideDeserialization { get; private set; }
        public bool OverrideSerializableTypeInfoAnalysis { get; private set; }
        #endregion

        #region Constructors
        protected SerializableAttribute(bool overrideSerialization, bool overrideDeserialization, bool overrideSerializableTypeInfoAnalysis) : base()
        {
            OverrideSerialization = overrideSerialization;
            OverrideDeserialization = overrideDeserialization;
            OverrideSerializableTypeInfoAnalysis = overrideSerializableTypeInfoAnalysis;
        }
        #endregion
    }
}
