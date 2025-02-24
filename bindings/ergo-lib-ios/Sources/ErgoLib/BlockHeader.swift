import Foundation
import ErgoLibC

/// Represents data of the block header available in Sigma propositions.
class BlockHeader {
    internal var pointer: BlockHeaderPtr
    
    /// Parse BlockHeader array from JSON (Node API)
    init(withJson json: String) throws {
        var ptr: BlockHeaderPtr?
        let error = json.withCString { cs in
            ergo_lib_block_header_from_json(cs, &ptr)
        }
        try checkError(error)
        self.pointer = ptr!
    }
    
    /// Takes ownership of an existing ``BlockHeaderPtr``. Note: we must ensure that no other instance
    /// of ``BlockHeader`` can hold this pointer.
    internal init(withRawPointer ptr: BlockHeaderPtr) {
        self.pointer = ptr
    }
    
    deinit {
        ergo_lib_block_header_delete(self.pointer)
    }
}

extension BlockHeader: Equatable {
    static func ==(lhs: BlockHeader, rhs: BlockHeader) -> Bool {
        ergo_lib_block_header_eq(lhs.pointer, rhs.pointer)
    }
}

/// An ordered collection of ``BlockHeader``s
class BlockHeaders {
    internal var pointer: BlockHeadersPtr
    
    /// Create an empty collection
    init() {
        self.pointer = BlockHeaders.initRawPtrEmpty()
    }
    /// Parse ``BlockHeader`` array from JSON (Node API)
    init(fromJSON: Any) throws {
        let json = JSON(fromJSON)
        if let arr = json.array {
            let headers = try arr.map{ elem -> BlockHeader in
                if let jsonStr = elem.rawString() {
                    return try BlockHeader(withJson: jsonStr);
                } else {
                    throw WalletError.walletCError(reason: "BlockHeaders.init(fromJSON): cannot cast array element to raw JSON string")
                }
            }
            self.pointer = BlockHeaders.initRawPtrEmpty()
            for header in headers {
                self.add(blockHeader: header)
            }
        } else {
            throw WalletError.walletCError(reason: "BlockHeaders.init(fromJSON): expected [JSON]")
        }
    }
    
    /// Use the C-API to create an empty collection and return the raw pointer that points to this
    /// collection.
    private static func initRawPtrEmpty() -> BlockHeaderPtr {
        var ptr: BlockHeadersPtr?
        ergo_lib_block_headers_new(&ptr)
        return ptr!
    }
    
    /// Return the length of the collection
    func len() -> UInt {
        return ergo_lib_block_headers_len(self.pointer)
    }
    
    /// Returns the ``BlockHeader`` at location `index` if it exists.
    func get(index: UInt) -> BlockHeader? {
        var blockHeaderPtr: BlockHeaderPtr?
        let res = ergo_lib_block_headers_get(self.pointer, index, &blockHeaderPtr)
        assert(res.error == nil)
        if res.is_some {
            return BlockHeader(withRawPointer: blockHeaderPtr!)
        } else {
            return nil
        }
    }
    
    /// Add a ``BlockHeader`` to the end of the collection.
    func add(blockHeader: BlockHeader) {
        ergo_lib_block_headers_add(blockHeader.pointer, self.pointer)
    }
        
    deinit {
        ergo_lib_block_headers_delete(self.pointer)
    }
}
