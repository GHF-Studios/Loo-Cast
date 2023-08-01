﻿using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface ISerializableAttribute
    {
        #region Methods
        public void Serialize(string attributeName, out XAttribute serializedAttribute);
        public void Deserialize(XAttribute serializedAttribute);
        #endregion
    }
}
