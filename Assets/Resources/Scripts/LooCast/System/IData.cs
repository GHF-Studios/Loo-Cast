namespace LooCast.System
{
    using System.Identification;
    
    public interface IData : IInstance
    {
        #region Properties
        IDataIdentifier DataIdentifier { get; }
        #endregion
    }
}