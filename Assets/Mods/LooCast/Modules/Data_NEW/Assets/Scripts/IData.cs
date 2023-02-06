namespace LooCast.Data_NEW
{
    using Identifier;
    
    public interface IData : IdentifierManager.IIdentifiableInstance
    {
        #region Properties
        IDataType DataType { get; }
        string Name { get; }
        #endregion
    }
}
