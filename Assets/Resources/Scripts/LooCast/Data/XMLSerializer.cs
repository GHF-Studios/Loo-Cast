﻿using System;

namespace LooCast.Data
{
    public class XMLSerializer : Serializer
    {
        #region Properties
        public override string ID
        {
            get
            {
                return "XML";
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
