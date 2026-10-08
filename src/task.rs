struct Task {
	id: u32,
	desc: String,
	completed: bool,  // Note: this might become something else? Like a "status" instead, but I don't see the point.
	priority: u8,     // Future-Proofing. Will not be used for a while.
	createdAt: str    // TODO: use custom Date struct
	deadlineAt: str   // TODO: use custom Date struct
}

