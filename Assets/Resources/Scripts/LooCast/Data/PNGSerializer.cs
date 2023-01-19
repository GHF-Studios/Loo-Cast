using System;

namespace LooCast.Data
{
    public class PNGSerializer : Serializer
    {
        #region Properties
        public override string ID
        {
            get
            {
                return "PNG";
            }
        }
        #endregion

        #region Methods
        public override T Deserialize<T>(string dataID)
        {
            
        }

        public override void Serialize<T>(T data)
        {

        }
        #endregion
    }
}
