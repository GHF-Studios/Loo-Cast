using System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceMetaDataIdentifiable : IInstanceMetaDataIdentifiable
    {
        #region Properties
        ICSharpInstanceMetaDataIdentifiable CSharpInstanceMetaDataIdentifier { get; }
        #endregion
    }
}
