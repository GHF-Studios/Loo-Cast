using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class SerializableFolderAttribute : SerializableAttribute
    {
        #region Constructors
        public SerializableFolderAttribute(bool overrideSerialization = false, bool overrideDeserialization = false, bool overrideSerializableTypeInfoAnalysis = false) : base(overrideSerialization, overrideDeserialization, overrideSerializableTypeInfoAnalysis)
        {
        }
        #endregion
    }
}
