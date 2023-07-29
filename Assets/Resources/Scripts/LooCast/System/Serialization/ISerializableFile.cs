using System;
using System.IO;

namespace LooCast.System.Serialization
{
    public interface ISerializableFile
    {
        #region Methods
        public void Serialize(out FileInfo serializedFile);
        public void Deserialize(FileInfo serializedFile);
        #endregion
    }
}
